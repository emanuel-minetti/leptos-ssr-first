- Add a breadcrumb to the layout
- Review `set_lang` server fn.
    (See: `The problem is not in the test but in server-side 'set_lang'`
    in `end2end/tests/i18n/user_lang.spec.ts`.)
    This seems to ba a problem relating to a lack of RAM.
    Nonetheless, it must be investigated: A server fn mustn't fail without reporting!
- Make Server Message testable (i.e., make it reloadable)
- Review mobile window width.
- Add a CI pipeline (maybe using GitHub actions)
  - Add apache as a reverse proxy to all environments (dev, test, prod)
- Retrieve session on reload from session storage if applicable
  - Remember to adapt e2e tests to use logout instead of reloading 
- Add modal to warn if login expires
- Optional: Get client configuration from the server
