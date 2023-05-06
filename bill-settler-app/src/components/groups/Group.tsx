import { useParams } from "@solidjs/router";
import SideBar from "../menu/SideBar";
import GroupContainer from "./GroupContainer";

export default function Group() {

    const params = useParams()

    return(
        <div class="flex flex-row h-screen">
            <SideBar/>
            <GroupContainer id={+(params.id)}/>
        </div>
    );
}