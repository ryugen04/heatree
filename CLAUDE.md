# Project Context
This is a TUI application for visualizing code metrics with heatmaps.

## Technology Stack
- Language: Rust/Go
- TUI Framework: Ratatui/Bubbletea
- Layout: Tree structure with heatmap visualization

## Development Rules
1. Never use goroutines directly with Bubbletea (use Commands instead)
2. Keep the event loop fast - all heavy computation in background
3. Use structured logging to debug.log (stdout is occupied by TUI)
4. Follow The Elm Architecture: Model -> Update -> View

## File Structure
src/
├── ui/          # UI components
├── data/        # Data processing
└── visualization/ # Heatmap rendering

## Testing
- Run with: cargo run / go run main.go
- Debug log: tail -f debug.log
- Visual verification required for layout changes

## Known Constraints
- TUI rendering requires exact layout calculations
- Color schemes must support both 256-color and truecolor terminals
- Screenshot comparison needed for UI accuracy
```

参照: https://www.anthropic.com/engineering/claude-code-best-practices

### 段階的開発アプローチ

#### **ステップ1: 計画フェーズ**
```
プロンプト例:
"UIスクリーンショットを分析して、このTUIアプリケーションの実装計画を立ててください。
1. 必要なコンポーネントをリストアップ
2. レイアウト構造を定義
3. データフローを設計
4. 実装の優先順位を決定

Think harder about the layout structure."
```

重要なキーワード:
- `think` / `think harder` / `ultrathink` - Claude Codeの拡張思考モードをトリガー
- 各レベルで思考バジェットが増加

#### **ステップ2: TDD(テスト駆動開発)**
```
プロンプト例:
"TDDアプローチで開発します。まず、以下の機能のテストを書いてください:
- ツリー構造の解析
- ヒートマップの色計算
- レイアウトのサイズ計算

テストが失敗することを確認してから、実装には進まないでください。"
```

参照: https://www.anthropic.com/engineering/claude-code-best-practices

#### **ステップ3: ビジュアルフィードバックループ**

**重要な問題点**: Claude Codeは標準ではTUIの視覚的な出力を確認できない

**解決策**:

1. **スクリーンショット駆動開発**
```
   プロンプト例:
   "TUIアプリを実行してスクリーンショットを撮影しました。
   [スクリーンショットをペースト]
   
   期待される出力と比較して、以下の問題を修正してください:
   - ツリー構造のインデントが不正確
   - ヒートマップの色が期待と異なる
   - レイアウトの幅が画面サイズに合っていない"
