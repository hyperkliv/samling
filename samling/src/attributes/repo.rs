use crate::deadpool_postgres::Object;
use samling_clorinde::client::{GenericClient, Params};

use super::{
    Attribute, AttributeType, CreateAttribute, CreateAttributeType, UpdateAttribute,
    UpdateAttributeType,
};
use crate::{
    auth::User,
    entity_ref::{ExternalIdEntity, Id, RefTarget},
    organizations::Organization,
    Error, Ref, Result, SlugEntity,
};
use samling_clorinde::queries::{
    attribute::{
        delete_attribute, get_attribute, insert_attribute, list_attributes, update_attribute,
        AttributeRow, AttributeRowBorrowed, InsertAttributeParams, UpdateAttributeParams,
    },
    attributetype::{
        delete_attribute_type, get_attribute_type, insert_attribute_type, list_attribute_types,
        update_attribute_type, AttributeTypeRow, AttributeTypeRowBorrowed,
        InsertAttributeTypeParams, UpdateAttributeTypeParams,
    },
};

#[derive(Clone)]
pub struct AttributesRepo;

impl AttributesRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Attribute>> {
        list_attributes()
            .bind(client, &organization_id.into())
            .map(handle_attribute_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Attribute>,
    ) -> Result<Option<Attribute>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if let Some(res) = get_attribute()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug.as_deref(),
            )
            .map(handle_attribute_row)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Attribute>,
    ) -> Result<Attribute> {
        if let Some(attribute) = self.find(client, organization_id, ref_).await? {
            Ok(attribute)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        attribute: CreateAttribute,
    ) -> Result<Attribute> {
        let tx = client.transaction().await?;
        let slug = attribute
            .check_or_generate_slug(&tx, organization_id, "")
            .await?;
        attribute
            .ensure_available_external_id(&tx, organization_id)
            .await?;
        let type_id = AttributeType::get_id(&tx, organization_id, &attribute.r#type).await?;
        let id: Id<Attribute> = insert_attribute()
            .params(
                &tx,
                &InsertAttributeParams {
                    title: attribute.title,
                    description: attribute.description,
                    slug: slug.as_ref(),
                    type_id: type_id.into(),
                    external_id: attribute.external_id.as_deref(),
                    organization_id: organization_id.into(),
                    created_by: created_by.into(),
                },
            )
            .one()
            .await?
            .into();

        tx.commit().await?;
        self.get(client, organization_id, &id.into()).await
    }

    pub async fn update(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        ref_: &Ref<Attribute>,
        attribute: UpdateAttribute,
    ) -> Result<Attribute> {
        let tx = client.transaction().await?;
        let existing = self.get(&tx, organization_id, ref_).await?;
        if attribute.slug.is_some() && Some(existing.slug) != attribute.slug {
            attribute
                .ensure_available_slug(&tx, organization_id)
                .await?;
        }
        if attribute.external_id.is_some() && existing.external_id != attribute.external_id {
            attribute
                .ensure_available_external_id(&tx, organization_id)
                .await?;
        }
        let type_id = if let Some(type_ref) = attribute.r#type {
            AttributeType::get_id(&tx, organization_id, &type_ref).await?
        } else {
            existing.r#type.id
        };
        let num_updated = update_attribute()
            .params(
                &tx,
                &UpdateAttributeParams {
                    title: attribute.title,
                    description: attribute.description,
                    type_id: Some(type_id.into()),
                    slug: attribute.slug.as_deref(),
                    external_id: attribute.external_id.as_deref(),
                    id: existing.id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_updated, 1);

        tx.commit().await?;

        self.get(client, organization_id, ref_).await
    }

    pub async fn upsert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        ref_: &Ref<Attribute>,
        mut attribute: CreateAttribute,
    ) -> Result<(bool, Attribute)> {
        attribute.update_external_id_from_ref(ref_);
        if Attribute::lookup_id(client, organization_id, ref_)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(client, organization_id, ref_, attribute.to_owned().into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, organization_id, created_by, attribute.to_owned())
                    .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Attribute>,
    ) -> Result<()> {
        let id = Attribute::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_attribute()
            .bind(client, &organization_id.into(), (&id).into())
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

fn handle_attribute_row(row: AttributeRowBorrowed) -> Result<Attribute> {
    let row: AttributeRow = row.into();
    Ok(Attribute {
        id: row.id.into(),
        title: serde_path_to_error::deserialize(row.title)?,
        description: serde_path_to_error::deserialize(row.description)?,
        r#type: serde_path_to_error::deserialize(row.r#type)?,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
    })
}

#[derive(Clone)]
pub struct AttributeTypesRepo;

impl AttributeTypesRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<AttributeType>> {
        list_attribute_types()
            .bind(client, &organization_id.into())
            .map(handle_attribute_type_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<AttributeType>,
    ) -> Result<Option<AttributeType>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if slug.is_some() {
            Err(Error::SlugReferenceUnsupported("attribute_type"))
        } else if let Some(res) = get_attribute_type()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug,
            )
            .map(handle_attribute_type_row)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<AttributeType>,
    ) -> Result<AttributeType> {
        if let Some(attribute_type) = self.find(client, organization_id, ref_).await? {
            Ok(attribute_type)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn insert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        attribute_type: CreateAttributeType,
    ) -> Result<AttributeType> {
        let tx = client.transaction().await?;
        let slug = attribute_type
            .check_or_generate_slug(&tx, organization_id, "")
            .await?;
        attribute_type
            .ensure_available_external_id(&tx, organization_id)
            .await?;
        let inserted_id: Id<AttributeType> = insert_attribute_type()
            .params(
                &tx,
                &InsertAttributeTypeParams {
                    name: attribute_type.name,
                    slug: slug.as_ref(),
                    external_id: attribute_type.external_id.as_deref(),
                    organization_id: organization_id.into(),
                    created_by: created_by.into(),
                },
            )
            .one()
            .await?
            .into();

        let attribute_type = self.get(&tx, organization_id, &inserted_id.into()).await?;
        tx.commit().await?;
        Ok(attribute_type)
    }

    pub async fn update(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        ref_: &Ref<AttributeType>,
        attribute_type: UpdateAttributeType,
    ) -> Result<AttributeType> {
        let existing = self.get(client, organization_id, ref_).await?;
        let tx = client.transaction().await?;
        if attribute_type.slug.is_some() && Some(existing.slug) != attribute_type.slug {
            attribute_type
                .ensure_available_slug(&tx, organization_id)
                .await?;
        }
        if attribute_type.external_id.is_some()
            && existing.external_id != attribute_type.external_id
        {
            attribute_type
                .ensure_available_external_id(&tx, organization_id)
                .await?;
        }
        let num_updated = update_attribute_type()
            .params(
                &tx,
                &UpdateAttributeTypeParams {
                    name: attribute_type.name,
                    slug: attribute_type.slug.as_deref(),
                    external_id: attribute_type.external_id.as_deref(),
                    id: existing.id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_updated, 1);

        let attribute_type = self.get(&tx, organization_id, ref_).await?;
        tx.commit().await?;
        Ok(attribute_type)
    }

    pub async fn upsert(
        &self,
        client: &mut Object,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        ref_: &Ref<AttributeType>,
        mut attribute_type: CreateAttributeType,
    ) -> Result<(bool, AttributeType)> {
        attribute_type.update_slug_from_ref(ref_);
        attribute_type.update_external_id_from_ref(ref_);
        if AttributeType::lookup_id(client, organization_id, ref_)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(
                    client,
                    organization_id,
                    ref_,
                    attribute_type.to_owned().into(),
                )
                .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(
                    client,
                    organization_id,
                    created_by,
                    attribute_type.to_owned(),
                )
                .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<AttributeType>,
    ) -> Result<()> {
        let id = AttributeType::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_attribute_type()
            .bind(client, &organization_id.into(), (&id).into())
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

fn handle_attribute_type_row(row: AttributeTypeRowBorrowed) -> Result<AttributeType> {
    let row: AttributeTypeRow = row.into();
    Ok(AttributeType {
        id: row.id.into(),
        name: serde_path_to_error::deserialize(row.name)?,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
    })
}
