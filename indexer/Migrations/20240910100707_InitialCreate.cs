using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Mumak.Indexer.Migrations
{
    /// <inheritdoc />
    public partial class InitialCreate : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.EnsureSchema(
                name: "public");

            migrationBuilder.CreateTable(
                name: "ReducerStates",
                schema: "public",
                columns: table => new
                {
                    Name = table.Column<string>(type: "text", nullable: false),
                    Slot = table.Column<decimal>(type: "numeric(20,0)", nullable: false),
                    Hash = table.Column<string>(type: "text", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_ReducerStates", x => x.Name);
                });

            migrationBuilder.CreateTable(
                name: "utxos",
                schema: "public",
                columns: table => new
                {
                    id = table.Column<string>(type: "text", nullable: false),
                    tx_index = table.Column<decimal>(type: "numeric(20,0)", nullable: false),
                    slot = table.Column<decimal>(type: "numeric(20,0)", nullable: false),
                    raw = table.Column<byte[]>(type: "bytea", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_utxos", x => new { x.slot, x.id, x.tx_index });
                });
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "ReducerStates",
                schema: "public");

            migrationBuilder.DropTable(
                name: "utxos",
                schema: "public");
        }
    }
}
