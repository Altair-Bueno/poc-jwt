<script lang="ts">
    import {authServClient} from "../config";
    import {credentialStore, getExpires} from "../credentials";
    import {displayBanner} from "../util";

    let basic = {username: "", password: ""}

    async function registerHandler() {
        try {
            await authServClient.authRegisterPost(basic)
            displayBanner("Register successful")
        } catch (exception) {
            displayBanner("Register exception", exception)
        }
    }

    async function loginHandler() {
        try {
            const response = await authServClient.authLoginPost(basic)
            const session = response.data
            console.log(session)
            // Special Svelte syntax to set dynamic stores
            $credentialStore = {
                accessToken: {
                    token: session.access_token,
                    expires: getExpires(session.expires_in)
                },
                refreshToken: session.refresh_token,
                username: basic.username
            }
            displayBanner("Login successful")
        } catch (exception) {
            displayBanner("Login exception", exception)
        }
    }

    async function checkAuthHandler() {
        try {
            await authServClient.authCheckGet()
            displayBanner("The user is authorised")
        } catch (exception) {
            displayBanner("The user is not authorised", exception)
        }
    }
</script>
<div>
  <h2>Authserver</h2>
  <p>Authserver is in charge of authentication. It also provides an endpoint
    to
    check if the user is authorised or not</p>
  <div>
  </div>
  <form on:submit|preventDefault>
    <div class="mb-3">
      <label class="form-label" for="username">Username</label>
      <input bind:value={basic.username} class="form-control" id="username"
             type="text">
    </div>
    <div class="mb-3">
      <label class="form-label" for="password">Password</label>
      <input bind:value={basic.password} class="form-control" id="password"
             type="text">
    </div>
  </form>

  <button class="btn btn-primary" on:click={registerHandler}>Register
  </button>
  <button class="btn btn-secondary" on:click={loginHandler}>Login</button>
  <button class="btn btn-info" on:click={checkAuthHandler}>
    Check if the user is authorised
  </button>
</div>