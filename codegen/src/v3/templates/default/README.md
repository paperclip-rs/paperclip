# Rust API client for {{{packageName}}}

{{#appDescriptionWithNewLines}}
{{{appDescriptionWithNewLines}}}
{{/appDescriptionWithNewLines}}

## Overview

- API version: {{{appVersion}}}
- Package version: {{{packageVersion}}}
{{^hideGenerationTimestamp}}
- Build date: {{{generatedDate}}}
{{/hideGenerationTimestamp}}
- Build package: {{{generatorClass}}}
{{#infoUrl}}
For more information, please visit [{{{infoUrl}}}]({{{infoUrl}}})
{{/infoUrl}}

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *{{{basePath}}}*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
{{#apiInfo}}{{#apis}}{{#operations}}{{#operation}}*{{{classname}}}* | [**{{{operationId}}}**]({{{apiDocPath}}}{{classname}}.md#{{{operationIdLowerCase}}}) | **{{{httpMethod}}}** {{{path}}} | {{#summary}}{{{summary}}}{{/summary}}
{{/operation}}{{/operations}}{{/apis}}{{/apiInfo}}

## Documentation For Models

{{#models}}{{#model}} - [{{{classname}}}]({{{modelDocPath}}}{{{classname}}}.md)
{{/model}}{{/models}}

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

{{#apiInfo}}{{#apis}}{{#-last}}{{{infoEmail}}}
{{/-last}}{{/apis}}{{/apiInfo}}