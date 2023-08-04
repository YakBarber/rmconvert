# rmconvert

*Bi-directional reMarkable conversion tooling.*

---

This repo contains tooling to perform bi-directional conversion with the [reMarkable tablet](https://remarkable.com)'s notebook file format, focusing on the *current* version of the format (firmware >3.0, rm lines file format v6). For information on the file format spec, see the [sister repo](https://github.com/YakBarber/remarkable_file_format).

The tooling is still very much a work in progress, but is minimally functional for certain conversion operations. It was conceived and developed to MVP during my 6-week batch at the [Recurse Center](https://www.recurse.com) in the summer of 2023.

Watch this space for updates. Happy to accept pull requests!

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
- [ ] **Non-hacky CLI**: it's jank right now
- [ ] **Baseline notebook access**: Looks in `~/sshrm`, which expects a sshfs (or copied) `/home/root/.local/share/remarkable/xochitl` directory from your tablet. Currently searches and finds the most recently modified page and automatically loads it.
- [ ] **More flexible notebook access**
- [ ] **`.metadata` reading/parsing**: currently only used to find modified time
- [ ] **`.content` reading/parsing**
- [ ] **`.pagefile` reading/parsing**
- [ ] **More docs**

