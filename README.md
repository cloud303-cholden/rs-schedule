### Configuration
```toml
# Schedule polling interval in milliseconds. Default is 100.
interval = 150         

# Base URL of API that will be invoked.
base_url = "http://localhost:3000"         

[[jobs]]               
# Multiple jobs can be configured.
# Route that is invoked when the job is triggered.
route = "/api/v1/job-1"   

# Default is to run once a day at midnight.
# Options are "rate" or "time".
# The `expr` is different for each `type`.
# For "rate", format is `<amount> <unit>`.
# Available <units> are "seconds", "minutes", "hours", "days".
# For "time", format is `<hour> <minute>`.
cron = { type = "rate", expr = "3 seconds" }
```
