interface Subdivision {
    name: string,
    id: string,
    area: [number, number][],
    lots: Lot[]
};

interface SubdivisionPreview {
    name: string,
    id: string,
    lots_amount: number
};

interface Lot {
    id: string,
    name: string,
    subdivision_id: string,
    area: [number, number][]
}