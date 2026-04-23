# Apollo

## Description

`Apollo` is a web-app, that helps with hackathon management.
It's easy to set up as a host and just as intuitive to use a contestant as well.

## Deployment process

1. The host sets up the server (see [`make help`](./Makefile) and [`.env.example`]).
2. Sets the `admin_password` (this will be used for encryption and authentication).
3. Adds puzzles to the apollo event.

And it's ready to take contestants, the hackathon can finally start!

## During the hackathon

1. Contestants join the event.
2. They submit their solutions
   - if it's correct, they get their points.
   - if it's incorrect, `apollo` reject the solution, obviously
3. Everyone can see the standings in real-time.

## Features

- Simplicity.
- State saving: to disk
- Customisability: see [`.env.example`], future admin interface
- Intuitive design: eg. session cookies
- Strong security: [see details and common pitfalls to watch out for](#Security).

## Security

- We use [`Argon2`] for password-hashing.
- Puzzle solutions are also stored as [`Argon2`] hashes (not raw plaintext).
- State saving is encrypted with [`chacha20poly1305`] (based on [this great guide])

> [!note]
> As a host pay attention to security and make sure to use `https`, as your hackathon contestants might have success cheating otherwise.

> [!tip]
> [`Certbot`] might be able to help you out getting `https` work.

> [!note]
> No rate-limiting is implemented, and anyway in general, it's advisable to put `apollo` behind a reverse-proxy, eg: [`nginx`] or [`traefik`].

## About

Development started in 2025 by [@csboo] and [@jarjk], two elder students of the [Lovassy László Gimnázium] to have a nice little manager for the famous in-house, annual hackathon: **Kódfejtő**.

## Caveats (i.e. `apollo` is a WIP)

- See [open issues].
- [`dioxus`] is in beta, so be patient during development.

## Code/Contributing

`Apollo` is written using [`dioxus`] (a React-like framework for/in [Rust]), styled with [`tailwindcss`].
To be able to contribute, [Rust] knowledge is most certainly necessary, go ahead and read the [amazing rustbook], afterward familirialise yourself with the [`dioxus` guide].
Also make sure to [open an issue] or reach out to us (somehow), before opening a PR ([here's a guide] for complete rookies) to make sure it aligns with our *unwritten* goals.
Definitely try to read the code and see whether you can understand it, we strive to write readable, easy-to-understand code.

[`apollo-cli.py`] is a manual CLI (mocker) client for testing the backend/server. See `./apollo-cli.py help`.

[`.env.example`]: ./.env.example
[`dioxus`]: https://dioxuslabs.com/
[Rust]: https://rust-lang.org
[`Argon2`]: https://en.wikipedia.org/wiki/Argon2
[`chacha20poly1305`]: https://en.wikipedia.org/wiki/ChaCha20-Poly1305
[this great guide]: https://kerkour.com/rust-file-encryption-chacha20poly1305-argon2
[`Certbot`]: https://certbot.eff.org/
[`nginx`]: https://nginx.org/en/
[`traefik`]: https://doc.traefik.io/traefik/
[@csboo]: https://github.com/csboo
[@jarjk]: https://github.com/jarjk
[Lovassy László Gimnázium]: https://web.lovassy.hu
[open issues]: https://github.com/csboo/apollo/issues/
[amazing rustbook]: https://doc.rust-lang.org/stable/book/
[open an issue]: https://github.com/csboo/apollo/issues/new/
[here's a guide]: https://docs.github.com/en/get-started/start-your-journey/about-github-and-git
[`dioxus` guide]: https://dioxuslabs.com/learn/0.7/
[`tailwindcss`]: https://tailwindcss.com/
[`apollo-cli.py`]: ./apollo-cli.py
