# rmconvert

*Bi-directional reMarkable conversion tooling.*

---

This repo contains tooling to perform bi-directional conversion with the [reMarkable tablet](https://remarkable.com)'s notebook file format, focusing on the *current* version of the format (firmware >3.0, rm lines file format v6). For information on the file format spec, see the [sister repo](https://github.com/YakBarber/remarkable_file_format).

The tooling is still very much a work in progress, but is minimally functional for certain conversion operations. It was originally conceived and developed to MVP during my 6-week batch at the [Recurse Center](https://www.recurse.com) in the summer of 2023.

Watch this space for updates. Happy to accept pull requests!

## Todos

Here are the current progress and targets. This plan will be further refined.

- [x] Implement `nom` parser for all structs
- [ ] SVG writing
    - [x] **Simple lines**
    - [ ] **Dynamic brush types**
    - [ ] **Line width**
    - [ ] **Color**
    - [ ] **Text**
    - [ ] **Templates(?)**
- [ ] SVG reading
    - [x] **Path M command**
    - [x] **Path L command**
    - [x] **Path C command**
    - [ ] **transforms**
        - [x] **matrix**
        - [ ] **???**
    - [ ] **Circles**
    - [ ] **Rectangles**
    - [ ] **Text**


- [ ] Extraction
    - [x] Lines to internal structs
    - [x] Text to internal structs
    - [ ] Output JSON
- [ ] Creation
- [~] **Insertion**: Sorta, through the lib


- [ ] **More flexible notebook access**
- [ ] **`.metadata` reading/parsing**: currently only used to find modified time
- [ ] **`.content` reading/parsing**
- [ ] **`.pagefile` reading/parsing**
- [ ] **More docs**: The Forever Todo

