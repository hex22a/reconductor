export type ProjectEntity = {
    id: string,
    owner_id: string,
    name: string,
    created_at: Date,
};

export type ProjectInsert = Pick<ProjectEntity, 'name' | 'owner_id'>;
