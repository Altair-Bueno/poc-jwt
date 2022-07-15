<script lang="ts">
  import { credentialStore } from "./lib/credentials";
  import SpringPing from "./lib/components/SpringPing.svelte";
  import AuthServ from "./lib/components/AuthServ.svelte";
  import { displayBanner } from "./lib/util";
  import AxumTransform from "./lib/components/AxumTransform.svelte";
  import { hasExpired } from "./lib/credentials.js";

  let expired;

  function accessTokenToClipboard() {
    const accessToken = $credentialStore?.accessToken.token;
    if (accessToken) {
      navigator.clipboard.writeText(accessToken);
    } else {
      displayBanner("The clipboard is empty", new Error());
    }
  }

  function refreshExpired() {
    expired = hasExpired($credentialStore?.accessToken.expires);
  }

  refreshExpired();
</script>

<div class="container">
  <div>
    <h1>Welcome to the sample JWT microservice frontend</h1>
    <p>The console contains more information about request/responses</p>
  </div>
  <AuthServ />
  <SpringPing />
  <AxumTransform />
  <footer class="bg-cornflowerblue">
    <h4>Stores content</h4>
    <div>
      access token: {$credentialStore?.accessToken.token} <br />
      refresh token: {$credentialStore?.refreshToken} <br />
      Expires: {$credentialStore?.accessToken.expires &&
        new Date($credentialStore?.accessToken.expires)}
      <br />
      Has expired? {expired}
      <button on:click={refreshExpired}>Check</button>
    </div>
    <button on:click={accessTokenToClipboard} class="btn btn-primary"
      >Copy access token to clipboard
    </button>
  </footer>
</div>
