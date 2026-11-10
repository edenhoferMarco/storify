### Date 
10.11.2026
### Who
Marco Edenhofer
### What 
Of all Names (e.g. SpaceName, FileName, DirectoryName), only UserName shall be a Value Object, since it is explicitly mentioned in the Bounded Context of the core domain (used for adding users to a Space).
### Tradeoffs
- Validation of the Entities who have the name as String may be a bit more tedious