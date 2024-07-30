interface BookMark {
    book_id: string;
    mark_id: number;
    start_position: Position;
    end_position: Position;
    create_time: number;
}

interface Position {
    chapter: number;
    paragraph: number;
    offset: number;
}

export type { BookMark, Position };
