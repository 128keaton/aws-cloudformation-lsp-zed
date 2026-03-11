# AWS Cloudformation LSP for Zed

This is a Language Server Protocol (LSP) implementation for AWS CloudFormation templates, designed to provide features such as diagnostics, code completion, and more within the Zed editor.

## Example Settings

```json
 "lsp": {
    "aws-cloudformation": {
      "settings": {
        "diagnostics": {
          "cfnLint": {
            "ignoreChecks": ["IAM_NO_INLINE_POLICY_CHECK"],
          },
        },
      },
      "initialization_options": {
        "aws": {
          "logLevel": "error",
          "telemetryEnabled": false,
        },
      },
    },
 }
```
