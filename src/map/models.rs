use crate::map::assests::SpawnableAssest;
use evy_precedural_tilemaps::prelude::*;

pub struct TerrainModelBuilder {
    pub models: ModelCollection<Cartesian3D>,
    pub assests: Vec<Vec<SpawnableAsset>>,
}
impl TerrainModelBuilder {
    pub fn new() -> Self {
        Self {
            models: ModelCollection::new(),
            assets: Vec::new(),
        }
    }

    pub fn create_model<T>(
        &mut self,
        template: T,
        assets: Vec<SpawnableAsset>,
    ) -> &mut Model<Cartesian3D>
    where
        T: Into<ModelTemplate<Cartesian3D>>,
    {
        let model_ref = self.models.create(template);
        self.assets.push(assets);
        model_ref
    }

    pub fn into_parts(self) -> (Vec<Vec<SpawnableAsset>>, ModelCollection<Cartesian3D>) {
        (self.assets, self.models)
    }
}
