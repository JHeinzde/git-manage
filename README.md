# Purpose 

This tool was borne, because managing a lot of local/remote branches that are protected
is a real pain in GitLab.

Automatically deleting merged branches which are protected does not work properly and 
also if it would it does no clean up your local workspace. 
I use a script called git-sync but this does not really solve this problem effectivly. This tool should
close the gap

# Build

This is a standard rust application so it can be built with ``crago build``.
Its dependencies are reqwest and serde to do json stuff.

# Configuration

This tool uses a configuration file that should be placed in your ````$HOME```` directory. The file should be called
```.helper_config.json``` and contains the following json object: 
```{
"api_token": "<my_token>",
"url": "<my_gitlab_url>",
"proxy": "<my_http_proxy>"
}
```

