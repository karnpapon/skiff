**NOTE** : private contribution would not be fetched, only public commit will be shown.

<div style="width: 100%; height: 0; padding-top: 56.25%; position: relative;">
  <iframe width="100%" height="100%" style="border:none;overflow:hidden;position:absolute;top:0;" src="https://www.youtube.com/embed/0MEqRyrv7BA" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
</div>

![img-1](/media/images/patithin/01.png)

| msg           | description                                                                                | type   | example               |
|---------------|--------------------------------------------------------------------------------------------|--------|-----------------------|
| module        | name of connecting module, currently support only   ER301                                  | String | er301                 |
| module_number | the order number of connecting module                                                      | Int    | 1                     |
| command       | command to send to module available command for ER301 documentation please refer to   this | String | cv_slew, tr_pulse, tr |
| output_port   | output port number                                                                         | Int    | 1                     |
| value         | value to send to                                                                           | Float  | 1000                  |