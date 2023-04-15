import SideBar from "../menu/SideBar";
import GroupsContainer from "./GroupsContainer";

export default function Groups() {
    return(
        <div class="flex flex-row h-screen">
            <SideBar/>
            <GroupsContainer/>
        </div>
    );
}