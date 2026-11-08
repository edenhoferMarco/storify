# Context

## Core Domain
### File Storage
```mermaid
flowchart TB
    Space --> Directory
    Space --> SpaceName
    Directory --> StoredFile
    Directory --> DirectoryToDelete
    Directory --> DirectoryName
    StoredFile --> FileInUpload
    StoredFile --> FileToDelete
    StoredFile --> FileName

    Space --> Member
    UserName
```

## Sub Domains
### User & Permissions
```mermaid
flowchart TB
    User --> UserName
    User --> Member
    Member --> Permission
    Permission --> Download
    Permission --> Upload
    Permission --> Own

```
