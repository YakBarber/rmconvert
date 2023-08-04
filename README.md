# rmconvert

*Bi-directional reMarkable conversion tooling.*

This repo contains tooling to perform bi-directionl conversion with the [reMarkable tablet](https://remarkable.com)'s notebook file format. 

The tooling is still very much a work in progress, but is minimally functional for certain conversion operations.

For information on the file format spec, see the [sister repo](https://github.com/YakBarber/remarkable_file_format).

## Todos

Here are the current progress and targets. This plan will be further refined.

- [ ] Implement `nom` parser for all structs
    - [x] **Line definition block**: Fully parsable and defined
    - [x] **Layer definition block**: Fully parsable, partially understood
    - [x] **Layer name block**: Fully parsable, partially understood
    - [x] **Layer information block**: Fully parsable, partially understood
    - [x] **Text definition block**: Fully parsable, partially understood
    - [ ] **Frontmatter**: Can be bypassed by seeking
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
    - [ ] **Circles**
    - [ ] **Rectangles**
    - [ ] **Text**

