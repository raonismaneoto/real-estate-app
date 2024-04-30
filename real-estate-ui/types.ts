interface Subdivision {
    name: string,
    id: string,
    area: [number, number][],
    lots: Lot[]
};

interface Lot {
    id: string,
    name: string,
    subdivision_id: string,
    area: [number, number][]
}