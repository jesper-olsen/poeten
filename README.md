# poeten
Poetry website - based on [yew](https://github.com/yewstack/yew) (single page webassembly app) and the [Rocket web framework](https://rocket.rs/).  

To run yew app standalone

```
% cd frontend
% trunk serve --open
```

To serve the app with Rocket (or [Axum](https://github.com/rksm/axum-yew-setup)): 
```
% cd frontend
% trunk build

% cd ../server_rocket
% cargo run
```
