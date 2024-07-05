using System;

class RemoteControlCar
{

    private int distance { get; set; } = 0;
    private int battery { get; set; } = 100;

    public static RemoteControlCar Buy() =>  new RemoteControlCar();

    public string DistanceDisplay() => $"Driven {this.distance} meters";

    public string BatteryDisplay()
    {
        if (this.battery > 0) {
            return $"Battery at {this.battery}%";
        } else {
            return "Battery empty";
        }
    }

    public void Drive()
    {
        if (this.battery > 0) {
            this.distance = this.distance + 20;
            this.battery = this.battery - 1;
        }
    }
}
