using HealthCheckApp.Models;
using System;
using System.Diagnostics;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;

namespace HealthCheckApp.Services
{
    public class HttpHealthCheckService : IHealthCheckService
    {
        private readonly IHttpClientFactory _httpClientFactory;

        public ApplicationType Type => ApplicationType.HTTP;

        public HttpHealthCheckService(IHttpClientFactory httpClientFactory)
        {
            _httpClientFactory = httpClientFactory ?? throw new ArgumentNullException(nameof(httpClientFactory));
        }

        public async Task<HealthCheckResult> CheckHealthAsync(ApplicationConfig appConfig)
        {
            var result = new HealthCheckResult(appConfig.Name);
            var stopwatch = new Stopwatch();

            try
            {
                var httpClient = _httpClientFactory.CreateClient();
                httpClient.Timeout = TimeSpan.FromSeconds(appConfig.TimeoutSeconds);

                var request = new HttpRequestMessage(new HttpMethod(appConfig.Method), appConfig.Target);

                if (appConfig.Headers != null)
                {
                    foreach (var header in appConfig.Headers)
                    {
                        request.Headers.TryAddWithoutValidation(header.Key, header.Value);
                    }
                }

                if (!string.IsNullOrEmpty(appConfig.RequestBody) && (appConfig.Method.ToUpper() == "POST" || appConfig.Method.ToUpper() == "PUT"))
                {
                    // Assuming headers specify content type, e.g., "Content-Type: application/json"
                    string contentType = "application/json"; // Default
                    if(appConfig.Headers != null && appConfig.Headers.TryGetValue("Content-Type", out var ctHeader))
                    {
                        contentType = ctHeader;
                    }
                    request.Content = new StringContent(appConfig.RequestBody, Encoding.UTF8, contentType);
                }

                stopwatch.Start();
                HttpResponseMessage response = await httpClient.SendAsync(request);
                stopwatch.Stop();

                result.ResponseTime = stopwatch.Elapsed;
                result.Status = response.StatusCode == (System.Net.HttpStatusCode)appConfig.ExpectedResponseCode
                                ? HealthStatus.Healthy
                                : HealthStatus.Unhealthy;

                if (result.Status == HealthStatus.Unhealthy)
                {
                    result.ErrorMessage = $"Unexpected status code: {response.StatusCode}. Response: {await response.Content.ReadAsStringAsync()}";
                }
            }
            catch (TaskCanceledException ex) // Catches timeouts
            {
                stopwatch.Stop();
                result.Status = HealthStatus.Unhealthy;
                result.ErrorMessage = $"Request timed out after {appConfig.TimeoutSeconds} seconds. {ex.Message}";
                result.ResponseTime = stopwatch.Elapsed; // Or TimeSpan.FromSeconds(appConfig.TimeoutSeconds)
            }
            catch (HttpRequestException ex)
            {
                stopwatch.Stop();
                result.Status = HealthStatus.Unhealthy;
                result.ErrorMessage = $"HTTP request failed: {ex.Message}";
                if(stopwatch.IsRunning) stopwatch.Stop();
                result.ResponseTime = stopwatch.Elapsed;
            }
            catch (Exception ex)
            {
                if(stopwatch.IsRunning) stopwatch.Stop();
                result.Status = HealthStatus.Unhealthy;
                result.ErrorMessage = $"An unexpected error occurred: {ex.Message}";
                result.ResponseTime = stopwatch.Elapsed;
            }
            finally
            {
                result.LastCheckedUtc = DateTime.UtcNow;
            }

            return result;
        }
    }
}
