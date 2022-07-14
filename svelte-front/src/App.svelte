<script lang="ts">
    import {authServClient, springPingClient} from "./lib/config";
    import {credentialStore, getExpires} from "./lib/credentials";

    let basic= {username: "", password: ""}
    let springPingContent = ''

    // Handlers
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

    async function springPingHandler() {
        try {
            const response = await springPingClient.rootPost(springPingContent)
            const data = response.data
            console.log(data)
            displayBanner(data)
        } catch (exception) {
            displayBanner("Ping error", exception)
        }
    }

    // Utility functions
    function displayBanner(message: string, exception?: Error) {
        const kind = exception ? 'Error' : 'Info'
        if (exception) console.log(exception)
        alert(`[${kind}]: ${message} \n${exception ? exception : ""}`)
    }
</script>
<style>
  /*Svelte also supports scoped styles*/
  .bg-cornflowerblue {
      background: cornflowerblue;
  }
</style>
<div class="container">
  <div>
    <h1>Welcome to the sample JWT microservice frontend</h1>
    <p>
      The console contains more information about request/responses
    </p>
  </div>

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
  <div>
    <h2>Spring-ping</h2>
    <p>Responds to an authorised user with the same content</p>
    <div>
      <textarea bind:value={springPingContent} class="form-control" type="text"></textarea>
      <button class="btn btn-primary" on:click={springPingHandler}>Send</button>
    </div>
  </div>
  <footer class="bg-cornflowerblue">
    <h4>Debug information</h4>
    <div>
      usernameField: {basic?.username} <br>
      passwordField: {basic?.password}
    </div>
    <div>
      access token: {$credentialStore?.accessToken.token} <br>
      refresh token: {$credentialStore?.refreshToken}
    </div>
  </footer>
</div>
