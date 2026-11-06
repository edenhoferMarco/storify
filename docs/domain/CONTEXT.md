# Context

## Core Domain
### File Storage
```mermaid
flowchart TB
    Space --> Directory
    Directory --> StoredFile
    Directory --> DirectoryToDelete
    StoredFile --> FileInUpload
    StoredFile --> FileToDelete

    Space --> Member
    UserName
```

## Sub Domains
### User & Permissions
```mermaid
flowchart TB
    User --> Member
    Member --> Permission
    Permission --> Download
    Permission --> Upload
    Permission --> Own

    User --> UserName
```
