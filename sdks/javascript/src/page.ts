import { type PageData } from './types';

export class Page implements PageData {
  url: string;
  markdown: string;
  links: string[];
  screenshot?: string;
  jsOutput?: string;

  constructor(data: PageData) {
    this.url = data.url;
    this.markdown = data.markdown;
    this.links = data.links;
    this.screenshot = data.screenshot;
    this.jsOutput = data.jsOutput;
  }

  getMainContent(): string {
    const lines = this.markdown.split('\n');
    const contentLines = lines.length > 2 ? lines.slice(2) : lines;
    return contentLines.join('\n').trim();
  }

  findLinksByText(text: string): string[] {
    const lowerText = text.toLowerCase();
    return this.links.filter(link => link.toLowerCase().includes(lowerText));
  }
}
