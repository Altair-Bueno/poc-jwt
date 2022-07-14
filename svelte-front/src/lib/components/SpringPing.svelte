<script lang="ts">
  import {springPingClient} from "../config";
  import {displayBanner} from "../util";

  let springPingContent = ''

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
</script>
<div>
  <h2>Spring-ping</h2>
  <p>Responds to an authorised user with the same content</p>
  <div>
    <textarea bind:value={springPingContent} class="form-control" type="text"></textarea>
    <button class="btn btn-primary" on:click={springPingHandler}>Send</button>
  </div>
</div>