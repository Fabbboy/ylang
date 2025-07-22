package tree_sitter_sable_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_sable "git.schaub-dev.xyz/cppuniverse/sable.git/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_sable.Language())
	if language == nil {
		t.Errorf("Error loading Sable grammar")
	}
}
