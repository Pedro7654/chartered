# chartered

a little dig at creating a private cargo repository with authenticated downloads, the plan is to have cargo connect to
a git server we setup that can serve a fake index generated just for the authenticated user that we can embed temporary
authentication credentials into.

learn more at https://book.chart.rs/

designed to be easily morphable into a first-class authenticated registry-provider once [one][1] [of][2] the cargo RFCs go
through.

[1]: https://github.com/rust-lang/rfcs/pull/2719
[2]: https://github.com/rust-lang/rfcs/pull/3139

[open tasks](https://github.com/w4/chartered/issues)

#### fine grained permissions per user per crate

- VISIBLE
- PUBLISH_VERSION
- YANK_VERSION
- MANAGE_USERS

#### organisation support

crates are required to be under an organisation, the organisation can be specified when declaring the custom registry
in `.cargo/config.toml` like so:

```
[registries]
my-org       = { index = "ssh://chart.rs:22/my-org" }
my-other-org = { index = "ssh://chart.rs:22/my-other-org" }
```

#### screen shots

<a href=".github/imgs/crate-view-light.webp"><img src=".github/imgs/crate-view-light.webp" alt="crate view (light)"></a>
<a href=".github/imgs/crate-view-dark.webp"><img src=".github/imgs/crate-view-dark.webp" alt="crate view (dark)"></a>
