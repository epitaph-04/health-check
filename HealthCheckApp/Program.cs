using HealthCheckApp.Components;
using HealthCheckApp.Services;
using HealthCheckApp.Hubs;
using Microsoft.AspNetCore.ResponseCompression;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddRazorComponents()
    .AddInteractiveServerComponents();

builder.Services.AddCors();
builder.Services.AddSingleton<ConfigurationService>();
builder.Services.AddHttpClient();
builder.Services.AddTransient<IHealthCheckService, HttpHealthCheckService>();
builder.Services.AddTransient<IHealthCheckService, DbHealthCheckService>();
builder.Services.AddHostedService<HealthCheckOrchestratorService>();

builder.Services.AddSignalR();
builder.Services.AddResponseCompression(opts =>
{
    opts.MimeTypes = ResponseCompressionDefaults.MimeTypes.Concat(
        new[] { "application/octet-stream" });
});

var app = builder.Build();

if (!app.Environment.IsDevelopment())
{
    app.UseExceptionHandler("/Error", createScopeForErrors: true);
}

app.UseResponseCompression();

app.UseCors(policy => policy.AllowAnyHeader().AllowAnyMethod().AllowAnyOrigin());
app.UseStaticFiles();
app.UseAntiforgery();
app.MapRazorComponents<App>()
    .AddInteractiveServerRenderMode();
app.MapBlazorHub();
app.MapHub<HealthCheckHub>("/healthcheckhub");

app.Run();
