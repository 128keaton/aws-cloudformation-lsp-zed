# AWS Cloudformation LSP for Zed

Uses my [fork](https://github.com/128keaton/cloudformation-languageserver) of AWS's Cloudformation LSP which patches one issue

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
