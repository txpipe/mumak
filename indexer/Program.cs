using Mumak.Indexer.Data;
using Cardano.Sync.Extensions;
using Cardano.Sync.Reducers;
using Cardano.Sync.Data.Models;
using Mumak.Indexer.Reducers;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

builder.Services.AddCardanoIndexer<MumakDbContext>(builder.Configuration);
builder.Services.AddSingleton<IReducer<IReducerModel>, UtxoReducer>();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}


app.Run();