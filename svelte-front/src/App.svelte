<script lang="ts">
  import { credentialStore } from "./lib/credentials";
  import SpringPing from "./lib/components/SpringPing.svelte";
  import AuthServ from "./lib/components/AuthServ.svelte";
  import { displayBanner } from "./lib/util";
  import AxumTransform from "./lib/components/AxumTransform.svelte";

  function accessTokenToClipboard() {
    const accessToken = $credentialStore?.accessToken.token;
    if (accessToken) {
      navigator.clipboard.writeText(accessToken);
    } else {
      displayBanner("The clipboard is empty", new Error());
    }
  }
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
      refresh token: {$credentialStore?.refreshToken}
    </div>
    <button on:click={accessTokenToClipboard} class="btn btn-primary"
      >Copy access token to clipboard
    </button>
  </footer>
</div>
