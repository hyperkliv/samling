export interface Settings {
  apiBaseUrl: string;
  googleClientId: string;
  microsoftClientId: string;
}

const settings: Settings = {
  apiBaseUrl: makeApiBaseUrl(),
  googleClientId: process.env.REACT_APP_GOOGLE_CLIENT_ID as string,
  microsoftClientId: process.env.REACT_APP_MICROSOFT_CLIENT_ID as string,
};

function makeApiBaseUrl(): string {
  let url = process.env.REACT_APP_API_BASE_URL as string;
  if (url.startsWith("/")) {
    const prefix = `${window.location.protocol}//${window.location.host}`;
    console.debug(
      `API base URL only contains the path ${url}. Prefixing with ${prefix}!`,
    );
    url = prefix + url;
  }
  return url;
}

export default settings;
