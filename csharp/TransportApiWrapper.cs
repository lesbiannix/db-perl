using System;
using System.Runtime.InteropServices;

public class TransportApiWrapper
{
    [DllImport("transport_api_wrapper")]
    private static extern IntPtr locations(string query);

    [DllImport("transport_api_wrapper")]
    private static extern void free_string(IntPtr s);

    public static string Locations(string query)
    {
        IntPtr ptr = locations(query);
        string result = Marshal.PtrToStringAnsi(ptr);
        free_string(ptr);
        return result;
    }

    public static void Main(string[] args)
    {
        string locations = Locations("berlin");
        Console.WriteLine(locations);
    }
}