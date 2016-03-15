import { Observable } from 'rx';

export function makeSceneRouter(self, globalProps) {
  globalProps = globalProps || {};
  return function getScene() {
    const scene = self.refs['scene'];
    return {
      scene,
      goto: (path, props, options) => {
        props = props || {};
        return scene.goto(path, Object.assign({}, props, globalProps), options);
      },
      goback: scene.goback.bind(scene)
    };
  }
}

export function makeGoto$(path, interaction$, scene$) {
  return interaction$
    .combineLatest(scene$, (_, scene) => scene().goto(path));
}

export function makeGoback$(interaction$, scene$) {
  return interaction$
    .combineLatest(scene$, (_, scene) => scene().goback());
}
