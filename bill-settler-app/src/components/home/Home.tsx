import SideBar from "../menu/SideBar";
import HomeContainer from "./HomeContainer";

export default function Home() {
    return(
        <div class="flex flex-row h-screen">
            <SideBar/>
            <HomeContainer/>
        </div>
    );
}