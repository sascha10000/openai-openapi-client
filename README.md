# OpenAI client for rust generated from the OpenAPI specification

This repository is a modified fork of the specification provided by [OpenAPI](https://www.openapis.org/). The original specification can be found [OpenAI API](https://platform.openai.com/docs/api-reference).

The client itself misses features which lead to errors in the generated client. Those modules are commented out and can be added manually and fixed if needed.

## Changes
The original lead to validation errors due to certain "default: null" fields. The details can be found in [issue](https://github.com/openai/openai-openapi/issues/133) #133.

## Command
```bash
openapi-generator-cli generate -i openapi.yaml -g rust -o rust-openai-client
```

## Missing features in the rust client
- audio_api
- completions_api
- embeddings_api
- files_api
- images_api

## Contribution
Feel free to contribute to this repository by opening a pull request or an issue.
