# Simple demo app with Actix Web microframework

The app exposes the `/qrcode` API endpoint which accepts a `content` string as query parameter.
It returns a PNG in the body with a QRCode that encodes the string.