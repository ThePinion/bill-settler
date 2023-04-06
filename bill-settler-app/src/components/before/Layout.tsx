import { Outlet, Route } from "@solidjs/router";
import Login from "./Login";

export default function BeforeLayout() {
    return <section class="hero is-fullheight">
    <div class="hero-body">
      <div class="container">
        <div class="columns is-centered">
            <div class="column is-half">
              <Outlet />
            </div>
        </div>
      </div>
    </div>
  </section>
}

