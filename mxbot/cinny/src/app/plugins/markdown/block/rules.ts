import { BlockMDRule } from './type';

const HEADING_REG_1 = /^(#{1,6}) +(.+)\n?/m;
export const HeadingRule: BlockMDRule = {
  match: (text) => text.match(HEADING_REG_1),
  html: (match, parseInline) => {
    const [, g1, g2] = match;
    const level = g1.length;
    return `<h${level} data-md="${g1}">${parseInline ? parseInline(g2) : g2}</h${level}>`;
  },
};

const CODEBLOCK_MD_1 = '```';
const CODEBLOCK_REG_1 = /^`{3}(\S*)\n((?:.*\n)+?)`{3} *(?!.)\n?/m;
export const CodeBlockRule: BlockMDRule = {
  match: (text) => text.match(CODEBLOCK_REG_1),
  html: (match) => {
    const [, g1, g2] = match;
    const classNameAtt = g1 ? ` class="language-${g1}"` : '';
    return `<pre data-md="${CODEBLOCK_MD_1}"><code${classNameAtt}>${g2}</code></pre>`;
  },
};

const BLOCKQUOTE_MD_1 = '>';
const QUOTE_LINE_PREFIX = /^> */;
const BLOCKQUOTE_TRAILING_NEWLINE = /\n$/;
const BLOCKQUOTE_REG_1 = /(^>.*\n?)+/m;
export const BlockQuoteRule: BlockMDRule = {
  match: (text) => text.match(BLOCKQUOTE_REG_1),
  html: (match, parseInline) => {
    const [blockquoteText] = match;

    const lines = blockquoteText
      .replace(BLOCKQUOTE_TRAILING_NEWLINE, '')
      .split('\n')
      .map((lineText) => {
        const line = lineText.replace(QUOTE_LINE_PREFIX, '');
        if (parseInline) return `${parseInline(line)}<br/>`;
        return `${line}<br/>`;
      })
      .join('');
    return `<blockquote data-md="${BLOCKQUOTE_MD_1}">${lines}</blockquote>`;
  },
};

const ORDERED_LIST_MD_1 = '-';
const O_LIST_ITEM_PREFIX = /^(-|[\da-zA-Z]\.) */;
const O_LIST_START = /^([\d])\./;
const O_LIST_TYPE = /^([aAiI])\./;
const O_LIST_TRAILING_NEWLINE = /\n$/;
const ORDERED_LIST_REG_1 = /(^(?:-|[\da-zA-Z]\.) +.+\n?)+/m;
export const OrderedListRule: BlockMDRule = {
  match: (text) => text.match(ORDERED_LIST_REG_1),
  html: (match, parseInline) => {
    const [listText] = match;
    const [, listStart] = listText.match(O_LIST_START) ?? [];
    const [, listType] = listText.match(O_LIST_TYPE) ?? [];

    const lines = listText
      .replace(O_LIST_TRAILING_NEWLINE, '')
      .split('\n')
      .map((lineText) => {
        const line = lineText.replace(O_LIST_ITEM_PREFIX, '');
        const txt = parseInline ? parseInline(line) : line;
        return `<li><p>${txt}</p></li>`;
      })
      .join('');

    const dataMdAtt = `data-md="${listType || listStart || ORDERED_LIST_MD_1}"`;
    const startAtt = listStart ? ` start="${listStart}"` : '';
    const typeAtt = listType ? ` type="${listType}"` : '';
    return `<ol ${dataMdAtt}${startAtt}${typeAtt}>${lines}</ol>`;
  },
};

const UNORDERED_LIST_MD_1 = '*';
const U_LIST_ITEM_PREFIX = /^\* */;
const U_LIST_TRAILING_NEWLINE = /\n$/;
const UNORDERED_LIST_REG_1 = /(^\* +.+\n?)+/m;
export const UnorderedListRule: BlockMDRule = {
  match: (text) => text.match(UNORDERED_LIST_REG_1),
  html: (match, parseInline) => {
    const [listText] = match;

    const lines = listText
      .replace(U_LIST_TRAILING_NEWLINE, '')
      .split('\n')
      .map((lineText) => {
        const line = lineText.replace(U_LIST_ITEM_PREFIX, '');
        const txt = parseInline ? parseInline(line) : line;
        return `<li><p>${txt}</p></li>`;
      })
      .join('');

    return `<ul data-md="${UNORDERED_LIST_MD_1}">${lines}</ul>`;
  },
};

export const UN_ESC_BLOCK_SEQ =
  /^\\*(#{1,6} +|```|>|(-|[\da-zA-Z]\.) +|\* +| *\|.+ *\n *\|[-:]+\|)/;
export const ESC_BLOCK_SEQ =
  /^\\(\\*(#{1,6} +|```|>|(-|[\da-zA-Z]\.) +|\* +| *\|.+ *\n *\|[-:]+\|))/;

const TABLE_MD_1 = '|';
const TABLE_REG_1 = /^((?: *\|.+ *\n)+)((?: *\|[-:]+\| *\n)?)(?: *\|.+ *\n)*/m;
const TABLE_HEADER_REG = /^ *\|(.+) *\| *\n?$/m;
const TABLE_SEPARATOR_REG = /^ *\|([-: ]+ *\|)+ *\n?$/m;
const TABLE_ROW_REG = /^ *\|(.+) *\| *\n?$/m;

export const TableRule: BlockMDRule = {
  match: (text) => {
    const match = text.match(TABLE_REG_1);
    if (!match) return null;
    const [, headerLines] = match;
    const headerLine = headerLines.split('\n')[0];
    if (!headerLine.includes('|')) return null;
    return match;
  },
  html: (match, parseInline) => {
    const fullText = match[0];
    const lines = fullText.trim().split('\n');

    if (lines.length < 2) return fullText;

    const headerLine = lines[0].trim();
    const separatorLine = lines[1].trim();
    const bodyLines = lines.slice(2);

    const headerMatch = headerLine.match(TABLE_HEADER_REG);
    if (!headerMatch) return fullText;

    const headerCells = headerMatch[1]
      .split('|')
      .map((cell) => cell.trim())
      .filter((cell) => cell !== '');

    const separatorMatch = separatorLine.match(TABLE_SEPARATOR_REG);
    if (!separatorMatch) return fullText;

    const alignments: string[] = [];
    const alignmentCells = separatorMatch[0]
      .split('|')
      .map((cell) => cell.trim())
      .filter((cell) => cell !== '');

    alignmentCells.forEach((cell) => {
      if (cell.startsWith(':') && cell.endsWith(':')) {
        alignments.push('center');
      } else if (cell.endsWith(':')) {
        alignments.push('right');
      } else {
        alignments.push('left');
      }
    });

    const headerHtml = headerCells
      .map((cell, index) => {
        const alignment = alignments[index] || 'left';
        return `<th align="${alignment}">${parseInline ? parseInline(cell) : cell}</th>`;
      })
      .join('');

    const bodyHtml = bodyLines
      .map((line) => {
        const rowMatch = line.match(TABLE_ROW_REG);
        if (!rowMatch) return '';
        const cells = rowMatch[1]
          .split('|')
          .map((cell) => cell.trim())
          .filter((cell) => cell !== '');
        return `<tr>${cells
          .map((cell, index) => {
            const alignment = alignments[index] || 'left';
            return `<td align="${alignment}">${parseInline ? parseInline(cell) : cell}</td>`;
          })
          .join('')}</tr>`;
      })
      .join('');

    return `<table data-md="${TABLE_MD_1}"><thead><tr>${headerHtml}</tr></thead><tbody>${bodyHtml}</tbody></table>`;
  },
};
