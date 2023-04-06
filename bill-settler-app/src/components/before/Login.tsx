export default function Login(){
  return <div>
    <div class="box">
      <h1 class="title has-text-centered">Login</h1>
      <form>
        <div class="field">
          <label class="label">Email</label>
          <div class="control">
              <input class="input" type="email" placeholder="Email" />
          </div>
        </div>

        <div class="field">
          <label class="label">Password</label>
              <div class="control">
          <input class="input" type="password" placeholder="Password"/>
          </div>
        </div>

        <div class="field">
          <label class="checkbox">
            <input type="checkbox" /> I agree to the <a href="#">terms and conditions</a>
          </label>
        </div>

        <div class="field">
          <div class="control">
              <button class="button is-primary is-fullwidth">Login</button>
          </div>
        </div>
      </form>
    </div>
    <div class="box">
      <label class="label has-text-weight-normal">Don't have an account?</label>
      <div class="control">
        <button class="button is-secondary">Register</button>
      </div>
    </div>
  </div>
}