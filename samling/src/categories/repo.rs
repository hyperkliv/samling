use samling_clorinde::client::{GenericClient, Params};

use super::{Category, CreateCategory, UpdateCategory};
use crate::auth::User;
use crate::entity_ref::{ExternalIdEntity, Id, Ref, RefTarget, SlugEntity};
use crate::organizations::Organization;
use crate::Result;
use samling_clorinde::queries::category::{
    delete_category, get_category, insert_category, list_categories, update_category, CategoryRow,
    CategoryRowBorrowed, DeleteCategoryParams, InsertCategoryParams, UpdateCategoryParams,
};

#[derive(Clone)]
pub struct CategoriesRepo;

impl CategoriesRepo {
    pub async fn list(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
    ) -> Result<Vec<Category>> {
        list_categories()
            // TODO: Implement pagination
            .bind(client, &organization_id.into())
            .map(handle_row)
            .all()
            .await?
            .into_iter()
            .collect::<Result<_>>()
    }

    pub async fn get(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Category>,
    ) -> Result<Category> {
        if let Some(category) = self.find(client, organization_id, ref_).await? {
            Ok(category)
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn find(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Category>,
    ) -> Result<Option<Category>> {
        let (id, external_id, slug) = ref_.to_owned().take_all_inner();
        if let Some(res) = get_category()
            .bind(
                client,
                &organization_id.into(),
                &id,
                &external_id.as_deref(),
                &slug.as_deref(),
            )
            .map(handle_row)
            .opt()
            .await?
        {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    pub async fn insert(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        category: CreateCategory,
    ) -> Result<Category> {
        let slug = category
            .check_or_generate_slug(client, organization_id, "")
            .await?;
        category
            .ensure_available_external_id(client, organization_id)
            .await?;
        let inserted_id: Id<Category> = insert_category()
            .params(
                client,
                &InsertCategoryParams {
                    organization_id: organization_id.into(),
                    slug: slug.as_ref(),
                    external_id: category.external_id.as_deref(),
                    name: category.name,
                    created_by: created_by.into(),
                },
            )
            .one()
            .await?
            .into();
        self.get(client, organization_id, &inserted_id.into()).await
    }

    pub async fn update(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Category>,
        category: UpdateCategory,
    ) -> crate::Result<Category> {
        if let Some(existing) = self.find(client, organization_id, ref_).await? {
            if category.slug.is_some() && Some(existing.slug) != category.slug {
                category
                    .ensure_available_slug(client, organization_id)
                    .await?;
            }
            if category.external_id.is_some() && existing.external_id != category.external_id {
                category
                    .ensure_available_external_id(client, organization_id)
                    .await?;
            }
            let num_updated = update_category()
                .params(
                    client,
                    &UpdateCategoryParams {
                        slug: category.slug.as_deref(),
                        external_id: category.external_id.as_deref(),
                        name: category.name,
                        id: existing.id.into(),
                    },
                )
                .await?;
            debug_assert_eq!(num_updated, 1);
            self.get(client, organization_id, ref_).await
        } else {
            Err(ref_.not_found_error())
        }
    }

    pub async fn upsert(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        created_by: Id<User>,
        category_ref: &Ref<Category>,
        mut category: CreateCategory,
    ) -> Result<(bool, Category)> {
        category.update_slug_from_ref(category_ref);
        category.update_external_id_from_ref(category_ref);
        if Category::lookup_id(client, organization_id, category_ref)
            .await?
            .is_some()
        {
            Ok((
                false,
                self.update(client, organization_id, category_ref, category.into())
                    .await?,
            ))
        } else {
            Ok((
                true,
                self.insert(client, organization_id, created_by, category)
                    .await?,
            ))
        }
    }

    pub async fn delete(
        &self,
        client: &impl GenericClient,
        organization_id: Id<Organization>,
        ref_: &Ref<Category>,
    ) -> Result<()> {
        let id = Category::get_id(client, organization_id, ref_).await?;
        let num_deleted = delete_category()
            .params(
                client,
                &DeleteCategoryParams {
                    organization_id: organization_id.into(),
                    id: id.into(),
                },
            )
            .await?;
        debug_assert_eq!(num_deleted, 1);
        Ok(())
    }
}

pub(crate) fn handle_row(row: CategoryRowBorrowed) -> Result<Category> {
    let row: CategoryRow = row.into();
    Ok(Category {
        id: row.id.into(),
        name: serde_path_to_error::deserialize(row.name)?,
        slug: row.slug.into(),
        external_id: row.external_id.map(|v| v.into()),
        updated_at: row.updated_at,
        created_at: row.created_at,
        created_by: row.created_by.map(|v| v.into()),
    })
}
