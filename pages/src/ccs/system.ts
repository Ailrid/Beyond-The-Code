import { System, type ViridApp } from "@virid/core";
import {
  LoadStorageMessage,
  SaveStorageMessage,
  ToggleArticleMessage,
  UpdateProgressMessage,
} from "./message";
import { ArticleComponent } from "./component";

export class ArticleSystem {
  @System({
    messageClass: LoadStorageMessage,
  })
  public static loadStorage(articles: ArticleComponent) {
      
  }

  @System({
    messageClass: SaveStorageMessage,
  })
  public static saveStorage(articles: ArticleComponent) {
      
  }

  @System()
  public static toggleArticle(
    message: ToggleArticleMessage,
    articles: ArticleComponent,
  ) {
      
  }

  @System()
  public static updateProgress(
    message: UpdateProgressMessage,
    articles: ArticleComponent,
  ) {
      
  }
}
export function bindArticleSystem(app: ViridApp) {
  app.register(ArticleSystem.loadStorage);
  app.register(ArticleSystem.saveStorage);
  app.register(ArticleSystem.toggleArticle);
  app.register(ArticleSystem.updateProgress);
}
