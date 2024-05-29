# rmconvert

*Bi-directional reMarkable conversion tooling.*

---

This repo contains tooling to perform bi-directional conversion with the [reMarkable tablet](https://remarkable.com)'s notebook file format, focusing on the *current* version of the format (firmware >3.0, rm lines file format v6). For information on the file format spec, see the [sister repo](https://github.com/YakBarber/remarkable_file_format).

The tooling is still very much a work in progress, but is minimally functional for certain conversion operations. It was originally conceived and developed to MVP during my 6-week batch at the [Recurse Center](https://www.recurse.com) in the summer of 2023.

Watch this space for updates. Happy to accept pull requests!

## Todos

Here are the current progress and targets. This plan will be further refined.

- [x] **Parser for internal structs** - _all structures in RM files can be read into some `rmconvert::types` type_
    - [x] Lines
    - [x] Text

- [ ] Tests
    - [X] RM parser
    - [ ] RM writer
    - [ ] SVG parser
    - [ ] SVG writer
    - [ ] Drawing creation
    - [ ] integration
    - [ ] CLI

- [ ] **Docs** - _The Forever Todo_

- [ ] **Config file format**

- [ ] **SVG write support**
    - [x] Simple lines
    - [ ] Brush types
    - [ ] Line width
    - [ ] Color
    - [ ] Text
    - [ ] Templates(?)

- [ ] **SVG read support**
    - [x] Path M command
    - [x] Path L command
    - [x] Path C command
    - [ ] transforms
        - [x] matrix
        - [ ] ??? - _there are others_
    - [ ] Circles
    - [ ] Rectangles
    - [ ] Text

- [ ] **RM write support**
    - [X] Data insertion - _can draw on an existing page_
    - [ ] Page insertion - _can create a new page in existing notebook_
    - [ ] Notebook creation - _can create a new notebook, incl metadata_
    - [x] Simple lines
    - [ ] Brush types
    - [ ] Line width
    - [ ] Color
    - [ ] Text
    - [ ] Text with formatting

- [ ] **Output JSON**

- [ ] **More flexible notebook access**
- [ ] **`.metadata` reading/parsing**: currently only used to find modified time
- [ ] **`.content` reading/parsing**
- [ ] **`.pagefile` reading/parsing**

