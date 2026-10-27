import { EventMessage } from "@virid/core";
import { type ArticleInfo } from "./component";

export class ToggleArticleMessage extends EventMessage {
  public article: ArticleInfo;
  constructor(article: ArticleInfo) {
    super();
    this.article = JSON.parse(JSON.stringify(article));
  }
}

export class UpdateProgressMessage extends EventMessage {
  constructor(
    public articleId: string,
    public progress: number,
  ) {
    super();
  }
}

export class LoadStorageMessage extends EventMessage {}

export class SaveStorageMessage extends EventMessage {}
