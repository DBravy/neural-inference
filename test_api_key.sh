#!/bin/bash
# Test if the OpenAI API key works

source .env

echo "Testing API key..."
echo "Key starts with: ${OPENAI_API_KEY:0:15}..."
echo "Key ends with: ...${OPENAI_API_KEY: -10}"
echo "Key length: ${#OPENAI_API_KEY}"
echo ""

echo "Making test API request..."
curl -s https://api.openai.com/v1/models \
  -H "Authorization: Bearer $OPENAI_API_KEY" | head -20

echo ""
echo "If you see model data above, your key works!"
echo "If you see an error about invalid key, generate a new key at:"
echo "https://platform.openai.com/api-keys"

