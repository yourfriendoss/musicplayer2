import Bar from "./Bar";
import ColumnListing from "./ColumnListing"
import Player from './player/Player';
import {useState} from "react";

export default function Songs() {
    const allSongs = useState<any[]>([]);
    const selectedTrackId = useState('');

    function shuffleSongs() {
        allSongs[1](
          allSongs[0]
            .map((value) => ({ value, sort: Math.random() }))
            .sort((a, b) => a.sort - b.sort)
            .map(({ value }) => value)
        );
    }

    return <>
        <div
            style={{
                display: 'flex',
                flexDirection: 'column',
                height: '110vh',
            }}
            className="bg-background text-textColor"
            >
                <Bar/>
                <ColumnListing cols={[
                        "title", 
                        "artist", 
                        "album", 
                        "bitRate", 
                        "suffix", 
                        "duration"
                    ]} 
                    sortedElement="title"
                    selectedTrackId={selectedTrackId}
                    allSongs={allSongs}
                ></ColumnListing>
                <Player
                    tracks={allSongs[0] || []}
                    trackForced={selectedTrackId[0]}
                    // eslint-disable-next-line react/jsx-no-bind
                    shuffle={shuffleSongs}
                />
        </div>
    </>;
}
