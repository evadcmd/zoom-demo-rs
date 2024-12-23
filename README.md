# zoom-demo-rs

an Introduction to Zoom OAuth with a Simple Demo

A rust axum version of https://github.com/zoom/zoom-oauth-sample-app

## Create an OAuth App on the Zoom App Marketplace

Sign in to the Zoom App Marketplace and [Create an OAuth App](https://marketplace.zoom.us/develop/create?source=devdocs).

Creating this app will generate your OAuth Client ID and Secret needed to install on your account and get an access token.

Copy these credentials and add them to your `.env` file at root folder.

Example:

```
client_id=1234567890
client_secret=13245678901234567890
redirect_uri=https://12345678.ngrok.io
```

### ngrok

You may need to publish your app so you could get the redirect_uri for local testing

https://zenn.dev/manase/articles/03df0e18c93755

### Add Scopes

Required Permissions (Scopes)

The following OAuth scopes are necessary to create a meeting:

1. meeting:write
   • Description: Allows the application to create, update, and delete meetings for the authenticated user.
   • Scope Type: User-level or account-level.

2. meeting:write:admin
   • Description: Allows the application to create, update, and delete meetings for any user in the account.
   • Scope Type: Account-level; requires admin privileges.

## Run this App

```
cargo r
```

access the auth endpoint

```
https://${your_app_domain}/zoom-auth
```

References:

https://developers.zoom.us/docs/api/using-zoom-apis/

https://developers.zoom.us/docs/integrations/oauth/
