import { Component, type ViridApp } from "@virid/core";
import { Responsive } from "@virid/vue";

export interface ArticleInfo {
  id: string;
  title: string;
  count: number;
  date: string;
  desc: string;
}

@Component()
export class ArticleComponent {
  @Responsive()
  public currentArticle: ArticleInfo | null = null;

  @Responsive()
  public progress: Map<string, number> = new Map();

  @Responsive()
  public articleList: ArticleInfo[] = [];
}

export function bindArticleComponent(app: ViridApp) {
  app.bind(ArticleComponent);
}
