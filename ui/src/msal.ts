// Microsoft authentication library
import { Configuration, PublicClientApplication } from "@azure/msal-browser";
import settings from "./settings";

export const msalConfig: Configuration = {
  auth: {
    clientId: settings.microsoftClientId,
  },
};

export const msalInstance = new PublicClientApplication(msalConfig);
