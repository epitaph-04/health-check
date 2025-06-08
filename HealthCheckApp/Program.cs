using HealthCheckApp.Components;
using HealthCheckApp.Services;
using HealthCheckApp.Hubs;
using Microsoft.AspNetCore.ResponseCompression;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddRazorComponents()
    .AddInteractiveServerComponents();

// Custom Services Registration START
builder.Services.AddSingleton<ConfigurationService>();
builder.Services.AddHttpClient(); // For IHttpClientFactory, used by HttpHealthCheckService
builder.Services.AddTransient<IHealthCheckService, HttpHealthCheckService>();
builder.Services.AddTransient<IHealthCheckService, DbHealthCheckService>();
builder.Services.AddHostedService<HealthCheckOrchestratorService>();

builder.Services.AddSignalR();
builder.Services.AddResponseCompression(opts =>
{
    opts.MimeTypes = ResponseCompressionDefaults.MimeTypes.Concat(
        new[] { "application/octet-stream" });
});
// Custom Services Registration END

var app = builder.Build();

// Configure the HTTP request pipeline.
if (!app.Environment.IsDevelopment())
{
    app.UseExceptionHandler("/Error", createScopeForErrors: true);
    // The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
    app.UseHsts();
}

app.UseHttpsRedirection();

// Response Compression Middleware - place early in pipeline
app.UseResponseCompression();

app.UseStaticFiles();
app.UseAntiforgery();

app.MapRazorComponents<App>()
    .AddInteractiveServerRenderMode();

// Custom Hub Mapping START
app.MapHub<HealthCheckHub>("/healthcheckhub");
// Custom Hub Mapping END

app.Run();
