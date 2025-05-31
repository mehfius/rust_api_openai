#!/bin/sh
ngrok config add-authtoken "$NGROK_AUTHTOKEN"
./$API_NAME &
exec ngrok http --url="$NGROK_URL" "$PORT"