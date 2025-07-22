import XCTest
import SwiftTreeSitter
import TreeSitterSable

final class TreeSitterSableTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_sable())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Sable grammar")
    }
}
